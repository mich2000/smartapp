import React,{useState} from 'react';
import api_functions from '../../api';
import {InputWithButton} from '../input';
import email_util from '../../email';
import { showError, showInfo } from '../../toast';

export default function Registration(props) {
    const [registrationForm,setRegistrationForm] = useState({
        email : "",
        token : "",
        password : "",
        confirmed_password : ""
    });

    function registration_okay() {
        if(!email_util.control_email(registrationForm.email)) {
            showError("The email is not right.");
            return false;
        }
        if(registrationForm.password === "") {
            showError("Password cannot be empty");
            return false;
        }
        if(registrationForm.password_confirm !== registrationForm.password) {
            showError("Password and confirm password aren't the same.");
            return false;
        }
        return true;
    }

    function send_request(value) {
        if(!email_util.control_email(value)) {
            showError("Email is not in the correct format.");
            return;
        }
        let opties = api_functions.method_post();
        opties.body = JSON.stringify({
            email : value
        });
        fetch(api_functions.get_api() + "/request/new/user", opties)
        .then((api_call) => {
            if(api_call.status === 200) {
                showInfo("Token has been sent to your email account😀.");
            } else {
                api_call.text()
                .then(text =>showInfo(text))
                .catch(() => {
                    showError("Could not send through the request.");
                });
            }
        }).catch(() => {
            showError("Could not send through the request.");
        });
    }

    function registration(submit_event) {
        submit_event.preventDefault();
        submit_event.stopPropagation();
        if(!registration_okay()) {
            return;
        }
        let opties = api_functions.method_post();
        opties.credentials='omit';
        opties.body = JSON.stringify({
            email : registrationForm.email,
            token : registrationForm.token,
            password : registrationForm.password,
            password_confirm : registrationForm.password_confirm
        });
        fetch(api_functions.get_api() + "/auth/register", opties)
        .then((api_call) => {
            api_call.json()
            .then((json_obj) => {
                if(api_call.status === 200) {
                    showInfo("Your account has been created😀.");
                    setRegistrationForm({email : "",token : "",password : "",confirmed_password : ""});
                } else {
                    showError(json_obj.error);
                }
            });
        }).catch((e) => {
            showError(`Could not send through the request. error: ${e}`);
        });
    }

    return (
        <div>
            <div className="m-3">
                <h2>Send token creation😀</h2>
                <InputWithButton name="Submit" type="email" valuePlaceholder="Email registration to get token" input_callback={(value) => send_request(value)}/>
            </div>
            <form onSubmit={(e) => registration(e)}>
                <h2>Registration</h2>
                <div className="form-group">
                    <label className="control-label">New email</label>
                    <input type="email" className="form-control" value={registrationForm.email} name="email" onChange={(e) => setRegistrationForm({email : e.target.value, token : registrationForm.token, password : registrationForm.password, password_confirm : registrationForm.confirmed_password})} required autoComplete="new-email"/>
                </div>
                <div className="form-group">
                    <label className="control-label">Token</label>
                    <input type="text" maxLength="4" className="form-control" value={registrationForm.token} name="token" onChange={(e) => setRegistrationForm({email : registrationForm.email, token : e.target.value, password : registrationForm.password, password_confirm : registrationForm.confirmed_password})} required/>
                </div>
                <div className="form-group">
                    <label className="control-label">New password</label>
                    <input type="password" className="form-control" value={registrationForm.password} name="password" onChange={(e) => setRegistrationForm({email : registrationForm.email, token : registrationForm.token, password : e.target.value, password_confirm : registrationForm.confirmed_password})} required autoComplete="new-password"/>
                </div>
                <div className="form-group">
                    <label className="control-label">Confirm new password</label>
                    <input type="password" className="form-control" value={registrationForm.confirm_password} name="confirmed_password" onChange={(e) => setRegistrationForm({email : registrationForm.email, token : registrationForm.token, password : registrationForm.password, password_confirm : e.target.value})} required autoComplete="new-password"/>
                </div>
                <input type="submit" className="btn btn-primary" value="Register"/>
            </form>
        </div>
    );
}