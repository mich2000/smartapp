import React from 'react';
import api_functions from '../api';
import Input from './input';
import email from '../email';

export default class Registration extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            email : "",
            token : "",
            password : "",
            confirmed_password : ""
        }
        this.log_error = this.log_error.bind(this);
        this.send_request = this.send_request.bind(this);
        this.change_handler = this.change_handler.bind(this);
        this.registration = this.registration.bind(this);
    }

    registration_okay() {
        if(!email.control_email(this.state.email)) {
            this.log_error("The email is not right.");
            return false;
        }
        if(this.state.password === "") {
            this.log_error("Password cannot be empty");
            return false;
        }
        if(this.state.confirmed_password !== this.state.password) {
            this.log_error("Password and confirm password aren't the same.");
            return false;
        }
        return true;
    }

    change_handler(event) {
        this.setState({[event.target.name] : event.target.value});
    }

    log_error(err_msg) {
        this.props.error_callback(err_msg);
    }

    send_request(input) {
        if(!email.control_email(input)) {
            alert("Email is not good.");
            return;
        }
        let opties = api_functions.method_post();
        opties.body = JSON.stringify({
            email : input
        });
        fetch(api_functions.get_api() + "/request/new/user", opties)
        .then((api_call) => {
            api_call.json()
            .then((json_obj) => {
                if(api_call.status === 200) {
                    alert("Token has been sent to your email account😀.");
                } else {
                    this.log_error(json_obj.error);
                }
            });
        }).catch(() => {
            this.log_error("Could not send through the request.");
        });
    }

    registration(submit_event) {
        if(!this.registration_okay()) {
            submit_event.preventDefault();
            submit_event.stopPropagation();
            return;
        }
        let opties = api_functions.method_post();
        opties.body = JSON.stringify({
            email : this.state.email,
            token : this.state.token,
            password : this.state.password,
            password_confirm : this.state.confirmed_password
        });
        fetch(api_functions.get_api() + "/auth/register", opties)
        .then((api_call) => {
            api_call.json()
            .then((json_obj) => {
                if(api_call.status === 200) {
                    alert("Your account has been created😀.");
                } else {
                    this.log_error(json_obj.error);
                }
            });
        }).catch((e ) => {
            this.log_error(`Could not send through the request. error: ${e}`);
        });
        submit_event.preventDefault();
        submit_event.stopPropagation();
    }

    render() {
        return (
            <div>
                <Input name="Submit" valuePlaceholder="Email registration to get token" input_callback={(e) => this.send_request(e)}/>
                <form className="col-md-6" onSubmit={(e) => this.registration(e)}>
                    <h2>Registration</h2>
                    <div className="form-group">
                        <label className="control-label">New email</label>
                        <input type="email" className="form-control" value={this.state.email} name="email" onChange={this.change_handler} required autoComplete="new-email"/>
                    </div>
                    <div className="form-group">
                        <label className="control-label">Token</label>
                        <input type="text" maxLength="4" className="form-control" value={this.state.token} name="token" onChange={this.change_handler} required/>
                    </div>
                    <div className="form-group">
                        <label className="control-label">New password</label>
                        <input type="password" className="form-control" value={this.state.password} name="password" onChange={this.change_handler} required autoComplete="new-password"/>
                    </div>
                    <div className="form-group">
                        <label className="control-label">Confirm new password</label>
                        <input type="password" className="form-control" value={this.state.confirm_password} name="confirmed_password" onChange={this.change_handler} required autoComplete="new-password"/>
                    </div>
                    <input type="submit" className="btn btn-primary" value="Register"/>
                </form>
            </div>
        );
    }
}