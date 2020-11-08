import React, {useState,useContext} from 'react';
import {InputWithButton} from '../input';
import email from '../../email';
import api_functions from '../../api';
import {AppContext} from '../../state';

export function ChangeEmail(props) {
    const [emailInput,setEmailInput] = useState('');
    const [token, setToken] = useState('');
    const [user,setUser] = useContext(AppContext);

    function send_request(input){
        if(!email.control_email(input)) {
            setError("Email is not in the correct format.");
            return;
        }
        let options = api_functions.method_post();
        options.body = JSON.stringify({
            email : input
        });
        fetch(api_functions.get_api() + "/request/new/email",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                setEmailInput(input);
                setMessage('The request has been sent to your email.');
            } else {
                api_call.text()
                .then(err => setError(err));
            }
        }).catch((e) => {
            setError(`Could not send through the request. error: ${e}`)
        });
    }

    function change_email(event) {
        event.preventDefault();
        event.stopPropagation();
        if(!email.control_email(emailInput)) {
            setError("Email is not in the correct format.");
            return;
        }
        let options = api_functions.method_put();
        options.body = JSON.stringify({
            token : token,
            email : emailInput
        });
        fetch(api_functions.get_api() + "/user/change/email",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                setMessage('Email has been changed');
                setUser({email : emailInput, loggedIn : true});
                setToken('');
                setEmailInput('');
            } else {
                api_call.text()
                .then(err => setError(err));
            }
        }).catch((e) => {
            setError(`Could not send through the request. error: ${e}`);
        });
    }

    function setMessage(message) {
        props.message_callback(message,false);
    }

    function setError(err) {
        props.message_callback(err,true);
    }

    return (
        <form onSubmit={e => change_email(e)}>
            <div className="input-group m-3">
                <p>Put your email under here and use the token we send you to change you password.</p>
                <InputWithButton name="Send change token" valuePlaceholder="Enter your email" input_callback={input => send_request(input)}/>
            </div>
            { emailInput !== '' && 
            <div className="m-3">
                <div className="form-group">
                    <input className="form-control" contentEditable="false" value={emailInput}/>
                </div>
                <div className="form-group">
                    <input className="form-control" maxLength="4" placeholder="Needed token" value={token} onChange={e => setToken(e.target.value)}/>
                </div>
                <div className="input-group-btn">
                    <button className="btn btn-default" type="submit">
                        Change email
                    </button>
                </div>
            </div> }
        </form>
    );
}