import React, { useState }  from 'react';
import email_util from '../../email';

export function Login(props) {
    const [email_input, setEmail] = useState("");
    const [password, setPassword] = useState("");

    function login(event) {
        event.preventDefault();
        event.stopPropagation();
        if(!email_util.control_email(email_input)) {
            props.message_callback("Email was not in the correct format.",true);
            return;
        }
        if(password === "") {
            props.message_callback("Given password was empty.",true);
            return;
        }
        props.login_callback({ email : email_input, password : password });
    }

    return (
        <form onSubmit={(e) => login(e)}>
            <div className="input-group">
                <input type="email" className="form-control" value={email_input} onChange={(e) => setEmail(e.target.value)} placeholder="Enter email" required/>
                <input type="password" className="form-control" value={password} onChange={(e) => setPassword(e.target.value)} placeholder="Enter password" required/>
                <div className="input-group-btn">
                    <button className="btn btn-default" type="submit">
                        Log in
                    </button>
                </div>
            </div>
        </form>
    );
}