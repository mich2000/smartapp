import React from 'react';
import api_functions from '../../api';
import email from '../../email';

export function DeleteProfile(props) {
    function change_forgotten_pwd(event) {
        event.preventDefault();
        event.stopPropagation();
        if(!email.control_email(props.email)) {
            setError("Email is not in the correct format.");
            return;
        }
        if(!window.confirm("Do really wish to delete your account?")) {
            return;
        }
        let currentPwd = window.prompt("Put in your current password to delete your account:");
        if (currentPwd === '') {
            setError("A password cannot be empty");
            return;
        }
        if(!window.confirm("Last WARNING! Are you really sure to delete you account?")) { 
            return;
        }
        let options = api_functions.method_delete();
        options.headers.Authorization = `Basic ${btoa(props.email + ':' + currentPwd)}`;
        fetch(api_functions.get_api() + "/user/delete/profile",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                props.logout();
            } else {
                api_call.text()
                .then(err => setError(err));
            }
        }).catch((e) => {
            setError(`Could not send through the request. error: ${e}`);
        });
    }

    function setError(err) {
        props.message_callback(err,true);
    }

    return (
        <form onSubmit={e => change_forgotten_pwd(e)}>
            <div className="m-3">
                <div className="input-group-btn">
                    <button className="btn btn-danger" type="submit">
                        Delete you're account
                    </button>
                </div>
            </div>
        </form>
    );
}