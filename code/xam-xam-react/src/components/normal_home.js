import React from 'react';
import Registration from './user/registration';
import {Login} from './user/login';
import api_functions from '../api';
import {ForgottenPassword} from './user/forgotten_pwd';

export function About() {
    return (
        <div>
            <h1>About</h1>
        </div>
    );
}

export default class UnauthenticatedHome extends React.Component {
    constructor(props) {
        super(props);
        this.state ={
            error : ""
        }
        this.log_error = this.log_error.bind(this);
        this.login = this.login.bind(this);
    }

    log_error(err_msg) {
        console.error(err_msg);
        this.setState({ error : err_msg });
    }

    login(login_params) {
        let options = api_functions.method_get();
        options.headers.Authorization = `Basic ${btoa(login_params.email + ':' + login_params.password)}`;
        fetch(api_functions.get_api() + "/auth/login",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                this.props.login_callback(login_params.email);
            } else {
                api_call.json()
                .then(json => console.log(json));
            }
        }).catch((e) => {
            this.log_error(`Could not send through the request. error: ${e}`);
        });
    }

    render() {
        return (
            <div>
                <h1>xam-xam</h1>
                <span className="font-weight-bold text-danger">{this.state.error}</span>
                <div id="accordion" className="col-sm-8 m-3">
                    <div className="card">
                        <div className="card-header" id="UserRegistration">
                            <h4 className="mb-0">
                                <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserRegistration" aria-expanded="true" aria-controls="collapseUserRegistration">Registration</button>
                            </h4>
                        </div>
                        <div id="collapseUserRegistration" className="collapse" aria-labelledby="UserRegistration" data-parent="#accordion">
                            <div className="card-body">
                                <Registration error_callback={(e) => this.log_error(e)}/>
                            </div>
                        </div>
                    </div>
                    <div className="card">
                        <div className="card-header" id="UserLogin">
                            <h4 className="mb-0">
                                <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserLogin" aria-expanded="true" aria-controls="collapseUserLogin">Login</button>
                            </h4>
                        </div>
                        <div id="collapseUserLogin" className="collapse" aria-labelledby="UserLogin" data-parent="#accordion">
                            <div className="card-body">
                                <Login error_callback={(e) => this.log_error(e)} login_callback={(login) => this.login(login)}/>
                            </div>
                        </div>
                    </div>
                    <div className="card">
                        <div className="card-header" id="ForgottenPwd">
                            <h4 className="mb-0">
                                <button className="btn btn-link" data-toggle="collapse" data-target="#collapseForgottenPwd" aria-expanded="true" aria-controls="collapseForgottenPwd">Retrieve forgotten password</button>
                            </h4>
                        </div>
                        <div id="collapseForgottenPwd" className="collapse" aria-labelledby="ForgottenPwd" data-parent="#accordion">
                            <div className="card-body">
                                <ForgottenPassword/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}