import React,{useState} from 'react';
import api_functions from '../../api';
import {InputWithButton} from '../input';
import email from '../../email';

export default function Registration(props) {
    const [registrationForm,setRegistrationForm] = useState({
        email : "",
        token : "",
        password : "",
        confirmed_password : ""
    });

    function registration_okay() {
        if(!email.control_email(registrationForm.email)) {
            log_msg("The email is not right.",false);
            return false;
        }
        if(registrationForm.password === "") {
            log_msg("Password cannot be empty",false);
            return false;
        }
        if(registrationForm.password_confirm !== registrationForm.password) {
            log_msg("Password and confirm password aren't the same.",false);
            return false;
        }
        return true;
    }

    function log_msg(message, isError) {
        props.message_callback(message,isError);
    }

    function send_request(value) {
        if(!email.control_email(value)) {
            log_msg("Email is not in the correct format.",true);
            return;
        }
        let opties = api_functions.method_post();
        opties.body = JSON.stringify({
            email : value
        });
        fetch(api_functions.get_api() + "/request/new/user", opties)
        .then((api_call) => {
            if(api_call.status === 200) {
                log_msg("Token has been sent to your email account😀.",false);
            } else {
                api_call.text()
                .then(text =>log_msg(text,true))
                .catch(() => {
                    log_msg("Could not send through the request.");
                });
            }
        }).catch(() => {
            log_msg("Could not send through the request.");
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
                    log_msg("Your account has been created😀.",false);
                    setRegistrationForm({email : "",token : "",password : "",confirmed_password : ""});
                } else {
                    log_msg(json_obj.error,true);
                }
            });
        }).catch((e) => {
            log_msg(`Could not send through the request. error: ${e}`,true);
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