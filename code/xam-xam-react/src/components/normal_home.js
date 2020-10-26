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
                <Registration error_callback={(e) => this.log_error(e)}/>
            </div>
        );
    }
}