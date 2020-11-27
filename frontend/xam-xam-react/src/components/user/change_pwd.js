import React, { useState } from 'react';
import api_functions from '../../api';
import email from '../../email';

export function ChangePwd(props) {
    const [currentPwd,setCurrentPwd] = useState('');
    const [newPwd,setNewPwd] = useState('');
    const [newPwdConfirm,setNewPwdConfirm] = useState('');

    function delete_profile(event) {
        event.preventDefault();
        event.stopPropagation();
        if(!email.control_email(props.email)) {
            setError("Email is not in the correct format.");
            return;
        }
        let options = api_functions.method_put();
        console.log(props.email);
        options.headers.Authorization = `Basic ${btoa(props.email + ':' + currentPwd)}`;
        options.body = JSON.stringify({
            password : newPwd,
            password_confirm : newPwdConfirm
        });
        fetch(api_functions.get_api() + "/user/change/password",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                setMessage('Password has been changed');
                setCurrentPwd('');
                setNewPwd('');
                setNewPwdConfirm('');
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
        <form onSubmit={e => delete_profile(e)}>
            <div className="m-3">
                <div className="form-group">
                    <input className="form-control" type="password" placeholder="Enter current password" value={currentPwd} onChange={e => setCurrentPwd(e.target.value)} required/>
                </div>
                <div className="form-group">
                    <input className="form-control" type="password" autoComplete="new-password" placeholder="New password" value={newPwd} onChange={e => setNewPwd(e.target.value)} required/>
                </div>
                <div className="form-group">
                    <input className="form-control" type="password" autoComplete="new-password" placeholder="New confirmed password" value={newPwdConfirm} onChange={e => setNewPwdConfirm(e.target.value)} required/>
                </div>
                <div className="input-group-btn">
                    <button className="btn btn-default" type="submit">
                        Change password
                    </button>
                </div>
            </div>
        </form>
    );
}