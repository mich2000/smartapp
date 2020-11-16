import React, {useState,useContext} from 'react';
import Registration from './user/registration';
import {Login} from './user/login';
import api_functions from '../api';
import {ForgottenPassword} from './user/forgotten_pwd';
import {AppContext} from '../state';

export function About() {
    return (
        <div className="col-sm-10">
            <h1>About</h1>
        </div>
    );
}

export default function UnauthenticatedHome() {
    const [error,setError] = useState({msg : '',error : false});
    const [user,setUser] = useContext(AppContext);

    function log_msg(msg,err) {
        setError({ msg : msg, error : err });
    }

    function login(login_params) {
        console.log(login_params);
        let options = api_functions.method_get();
        options.headers.Authorization = `Basic ${btoa(login_params.email + ':' + login_params.password)}`;
        fetch(api_functions.get_api() + "/auth/login",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                setUser({email : login_params.email, loggedIn : user.loggedIn});
            } else {
                api_call.text()
                .then(err => setError(err,true));
            }
        }).catch((e) => setError(`Could not send through the request. error: ${e}`,true));
    }

    return (
        <>
            <h1>xam-xam</h1>
            <span className="font-weight-bold text-danger">{error.msg}</span>
            {
                (error.msg !== '' && error.error) ? <span className="font-weight-bold text-danger">{error.msg}</span> : <span className="font-weight-bold text-success">{error.msg}</span>
            }
            <div id="accordion" className="m-3">
                <div className="card text-center">
                    <div className="card-header" id="UserRegistration">
                        <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserRegistration" aria-expanded="true" aria-controls="collapseUserRegistration">Registration</button>
                    </div>
                    <div id="collapseUserRegistration" className="collapse" aria-labelledby="UserRegistration" data-parent="#accordion">
                        <div className="card-body">
                            <Registration message_callback={(e,status) => log_msg(e,status)}/>
                        </div>
                    </div>
                </div>
                <div className="card text-center">
                    <div className="card-header" id="UserLogin">
                        <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserLogin" aria-expanded="true" aria-controls="collapseUserLogin">Login</button>
                    </div>
                    <div id="collapseUserLogin" className="collapse" aria-labelledby="UserLogin" data-parent="#accordion">
                        <div className="card-body">
                            <Login message_callback={(e,s) => log_msg(e,s)} login_callback={(email) => login(email)}/>
                        </div>
                    </div>
                </div>
                <div className="card text-center">
                    <div className="card-header" id="ForgottenPwd">
                       <button className="btn btn-link" data-toggle="collapse" data-target="#collapseForgottenPwd" aria-expanded="true" aria-controls="collapseForgottenPwd">Retrieve forgotten password</button>
                    </div>
                    <div id="collapseForgottenPwd" className="collapse" aria-labelledby="ForgottenPwd" data-parent="#accordion">
                        <div className="card-body">
                            <ForgottenPassword message_callback={(e,s) => log_msg(e,s)}/>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}