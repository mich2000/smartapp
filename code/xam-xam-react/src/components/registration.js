import React from 'react';
import api_functions from '../api';
import Input from './input';
import email from '../email';

export default class Registration extends React.Component {
    constructor(props) {
        super(props);
        this.log_error = this.log_error.bind(this);
        this.send_request = this.send_request.bind(this);
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
            let response_body = api_call.json();
            if(api_call.status === 200) {
                alert("Token has been sent to your email account😀.");
            } else {
                this.log_error(response_body.error);
            }
        }).catch(() => {
            this.log_error("Could not send through the request.");
        });
    }

    render() {
        return (
            <div>
                <Input name="Submit" valuePlaceholder="Email registration to get token" input_callback={(e) => this.send_request(e)}/>
            </div>
        );
    }
}