import React from 'react';
import api_functions from '../../api';
import {InputWithButton} from '../input';
import email from '../../email';

export default class Registration extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            email : "",
            token : "",
            password : "",
            confirmed_password : ""
        }
        this.log_msg = this.log_msg.bind(this);
        this.send_request = this.send_request.bind(this);
        this.change_handler = this.change_handler.bind(this);
        this.registration = this.registration.bind(this);
    }

    registration_okay() {
        if(!email.control_email(this.state.email)) {
            this.log_msg("The email is not right.",false);
            return false;
        }
        if(this.state.password === "") {
            this.log_msg("Password cannot be empty",false);
            return false;
        }
        if(this.state.confirmed_password !== this.state.password) {
            this.log_msg("Password and confirm password aren't the same.",false);
            return false;
        }
        return true;
    }

    change_handler(event) {
        this.setState({[event.target.name] : event.target.value});
    }

    log_msg(message, isError) {
        this.props.message_callback(message,isError);
    }

    send_request(value) {
        if(!email.control_email(value)) {
            this.log_msg("Email is not in the correct format.",true);
            return;
        }
        let opties = api_functions.method_post();
        opties.body = JSON.stringify({
            email : value
        });
        fetch(api_functions.get_api() + "/request/new/user", opties)
        .then((api_call) => {
            api_call.json()
            .then((json_obj) => {
                if(api_call.status === 200) {
                    this.log_msg("Token has been sent to your email account😀.",false);
                } else {
                    this.log_msg(json_obj.error,true);
                }
            });
        }).catch(() => {
            this.log_msg("Could not send through the request.");
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
                    this.log_msg("Your account has been created😀.",false);
                    this.setState({email : "",token : "",password : "",confirmed_password : ""});
                } else {
                    this.log_msg(json_obj.error,true);
                }
            });
        }).catch((e) => {
            this.log_msg(`Could not send through the request. error: ${e}`,true);
        });
        submit_event.preventDefault();
        submit_event.stopPropagation();
    }

    render() {
        return (
            <div>
                <div className="m-3">
                    <h2>Send token creation😀</h2>
                    <InputWithButton name="Submit" type="email" valuePlaceholder="Email registration to get token" input_callback={(event,value) => this.send_request(event, value)}/>
                </div>
                <form onSubmit={(e) => this.registration(e)}>
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