import React, {useState} from 'react';
import {InputWithButton} from '../input';
import email from '../../email';
import api_functions from '../../api';

export function ForgottenPassword(props) {
    const [emailInput,setEmailInput] = useState('');
    const [token, setToken] = useState('');
    const [pwd, setPwd] = useState('');
    const [pwdConfirm, setPwdConfirm] = useState('');

    function send_request(input){
        if(!email.control_email(input)) {
            this.log_error("Email is not in the correct format.");
            return;
        }
        let options = api_functions.method_post();
        options.body = JSON.stringify({
            email : input
        });
        fetch(api_functions.get_api() + "/request/forgotten/pwd",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                setEmailInput(input);
            } else {
                console.log(api_call.body)
            }
        }).catch((e) => {
            console.err(`Could not send through the request. error: ${e}`);
        });
    }

    function change_forgotten_pwd(event) {
        event.preventDefault();
        event.stopPropagation();
        if(!email.control_email(emailInput)) {
            this.log_error("Email is not in the correct format.");
            return;
        }
        let options = api_functions.method_post();
        options.body = JSON.stringify({
            email : emailInput,
            token : token,
            password : pwd,
            password_confirm : pwdConfirm
        });
        fetch(api_functions.get_api() + "/auth/change/forgotten/pwd",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                alert('Password has been changed');
            } else {
                console.log(api_call.body)
            }
        }).catch((e) => {
            console.err(`Could not send through the request. error: ${e}`);
        });
    }

    return (
        <form onSubmit={e => change_forgotten_pwd(e)}>
            <div className="input-group m-3">
                <p>Put your email under here and use the token we send you to change you password.</p>
                <InputWithButton name="Retrieve password" valuePlaceholder="Enter your email" input_callback={(e) => send_request(e)}/>
            </div>
            { emailInput != '' && 
            <div className="m-3">
                <div className="form-group">
                    <input className="form-control" placeholder="Email" value={emailInput} contentEditable="false" />
                </div>
                <div className="form-group">
                    <input className="form-control" placeholder="Needed token" value={token} onChange={e => setToken(e.target.value)}/>
                </div>
                <div className="form-group">
                    <input className="form-control" placeholder="New password" value={pwd} onChange={e => setPwd(e.target.value)}/>
                </div>
                <div className="form-group">
                    <input className="form-control" placeholder="New confirmed assword" value={pwdConfirm} onChange={e => setPwdConfirm(e.target.value)}/>
                </div>
                <div className="input-group-btn">
                    <button className="btn btn-default" type="submit">
                        Change password
                    </button>
                </div>
            </div> }
        </form>
    );
}