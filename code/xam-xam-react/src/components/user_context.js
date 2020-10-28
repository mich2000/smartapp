import React from 'react';
import api_functions from '../api';
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";
import UnauthenticatedHome, {About} from './normal_home';
import {User} from './user';

export default class UserContext extends React.Component {
    constructor() {
        super();
        this.state = {
            logged_in : false
        }
        this.login = this.login.bind(this);
        this.logout = this.logout.bind(this);
        this.fetch_basic_info = this.fetch_basic_info.bind(this);
    }

    login() {
        this.setState({logged_in : true});
    }

    async fetch_basic_info() {
        let options = api_functions.method_get();
        await fetch(api_functions.get_api() + "/user/basic/info",options)
        .then((api_call) => {
            if(api_call.status === 200) {
                this.setState({logged_in : false});
                api_call.json()
                .then(json => console.log(json));
            } else {
                api_call.json()
                .then(json => console.log(json));
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    }

    componentDidMount() {
        this.fetch_basic_info();
    }

    logout() {
        fetch(api_functions.get_api + "/user/logout")
        .then(() => this.setState({logged_in : false}))
        .catch((e) => console.error("Could not log out. Error: " + e))
    }
    
    render() {
        if(!this.state.logged_in) {
            return (
                <Router>
                    <div className="col-sm-10">
                        <nav className="navbar navbar-expand-lg navbar-light bg-light">
                            <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
                                <span className="navbar-toggler-icon"></span>
                            </button>
                            <div className="collapse navbar-collapse" id="navbarNav">
                                <ul className="navbar-nav">
                                    <li className="nav-item">
                                        <Link className="nav-link" to="/">Home</Link>
                                    </li>
                                    <li className="nav-item">
                                        <Link className="nav-link" to="/about">About</Link>
                                    </li>
                                </ul>
                            </div>
                        </nav>
                        <Switch>
                            <Route path="/about">
                                <About />
                            </Route>
                            <Route path="/">
                                <UnauthenticatedHome login_callback={this.login}/>
                            </Route>
                        </Switch>
                    </div>
                    <span className="font-weight-bold text-danger">{this.state.error}</span>
                </Router>
            );
        }
        return (
            <Router>
                <div className="col-sm-10">
                    <nav className="navbar navbar-expand-lg navbar-light bg-light">
                        <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
                            <span className="navbar-toggler-icon"></span>
                        </button>
                        <div className="collapse navbar-collapse" id="navbarNav">
                            <ul className="navbar-nav">
                                <li className="nav-item">
                                    <Link className="nav-link" to="/">Home</Link>
                                </li>
                                <li className="nav-item">
                                    <Link className="nav-link" to="/about">About</Link>
                                </li>
                            </ul>
                        </div>
                    </nav>
                </div>
                <Switch>
                    <Route path="/about">
                        <About />
                    </Route>
                    <Route path="/">
                        <User/>
                    </Route>
                </Switch>
                <div className="div-inline-block">
                    <button className="btn btn-primary" onClick={this.logout}>
                        Log Out
                    </button>
                </div>
            </Router>
        );
    }
}