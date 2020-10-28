import React from 'react';
import Registration from './registration';

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
    }

    log_error(err_msg) {
        console.error(err_msg);
        this.setState({ error : err_msg });
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

                            </div>
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}