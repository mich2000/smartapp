import React, {useContext} from 'react';
import Registration from './user/registration';
import {Login} from './user/login';
import api_functions from '../api';
import {ForgottenPassword} from './user/forgotten_pwd';
import {AppContext} from '../state';
import {showError} from '../toast';

export function About() {
    return (
        <div className="col-sm-10">
            <h1>About</h1>
            <p>
                This is an application you can use to manage your food products in places where you store them. The food can be categorized by these places, for example fridge or a drawer where you hide cookies from your family.
            </p>
        </div>
    );
}

export default function UnauthenticatedHome() {
    const [user,setUser] = useContext(AppContext);

    function login(login_params) {
        let options = api_functions.method_get();
        options.headers.Authorization = `Basic ${btoa(login_params.email + ':' + login_params.password)}`;
        fetch(api_functions.get_api() + "/auth/login",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.text()
                .then(text => {
                    if(text === "authenticated") {
                        setUser({email : login_params.email, loggedIn : user.loggedIn});
                    } else {
                        showError(text);
                    }
                });
            } else {
                api_call.text()
                .then(err => showError(err));
            }
        }).catch(() => showError('No internet connection'));
    }

    return <>
            <h1>xam-xam</h1>
            <div id="accordion" className="m-3">
                <div className="card text-center">
                    <div className="card-header" id="UserRegistration">
                        <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserRegistration" aria-expanded="true" aria-controls="collapseUserRegistration">Registration</button>
                    </div>
                    <div id="collapseUserRegistration" className="collapse" aria-labelledby="UserRegistration" data-parent="#accordion">
                        <div className="card-body">
                            <Registration/>
                        </div>
                    </div>
                </div>
                <div className="card text-center">
                    <div className="card-header" id="UserLogin">
                        <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserLogin" aria-expanded="true" aria-controls="collapseUserLogin">Login</button>
                    </div>
                    <div id="collapseUserLogin" className="collapse" aria-labelledby="UserLogin" data-parent="#accordion">
                        <div className="card-body">
                            <Login login_callback={(email) => login(email)}/>
                        </div>
                    </div>
                </div>
                <div className="card text-center">
                    <div className="card-header" id="ForgottenPwd">
                       <button className="btn btn-link" data-toggle="collapse" data-target="#collapseForgottenPwd" aria-expanded="true" aria-controls="collapseForgottenPwd">Retrieve forgotten password</button>
                    </div>
                    <div id="collapseForgottenPwd" className="collapse" aria-labelledby="ForgottenPwd" data-parent="#accordion">
                        <div className="card-body">
                            <ForgottenPassword/>
                        </div>
                    </div>
                </div>
            </div>
        </>;
}