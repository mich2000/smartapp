import React from 'react';
import api_functions from '../api';
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";
import UnauthenticatedHome, {About} from './normal_home';

export default class UserContext extends React.Component {
    constructor() {
        super();
        this.state = {
            logged_in : false
        }
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
                                <UnauthenticatedHome/>
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

                    </Route>
                </Switch>
                <span className="font-weight-bold text-danger">{this.state.error}</span>
            </Router>
        );
    }
}