import React,{useContext, useEffect} from 'react';
import api_functions from '../api';
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";
import UnauthenticatedHome, {About} from './normal_home';
import {User} from './user/user';
import {AppContext} from '../state';
import {Storage} from './storage/storage';

export default function UserContext() {
    const [user,setUser] = useContext(AppContext);

    function login(email) {
        setUser({email : email, loggedIn : true});
    }

    function logout() {
        fetch(api_functions.get_api() + "/auth/logout",api_functions.method_get())
        .then(() => setUser({email : '', loggedIn : false}))
        .catch((e) => console.error("Could not log out. Error: " + e));
    }

    useEffect(() => {
        fetch(api_functions.get_api() + "/auth/validate",api_functions.method_get())
        .then((api_call) => {
            if(api_call.status === 200) {
                setUser({email : user.email, loggedIn: true});
                fetch(api_functions.get_api() + "/auth/renew/token",api_functions.method_get())
                .then((api_call) => {
                    if(api_call.status !== 200) {
                        api_call.text()
                        .then((text) => console.log(text))
                        .catch((e) => console.error(`Could not send through the request. error: ${e}`));
                    }
                })
                .catch((e) => console.error(`Could not send through the request. error: ${e}`));
            } else {
                api_call.text()
                .then((text) => console.log(text))
                .catch((e) => console.error(`Could not send through the request. error: ${e}`));
            }
        })
        .catch((e) => console.error(`Could not send through the request. error: ${e}`));
    },[user.email])

    function render() {
        if(!user.loggedIn) {
            return (
                <Router>
                    <div className="col-sm-10">
                        <nav className="navbar navbar-expand-lg navbar-light bg-light">
                            <button className="navbar-toggler" type="button" data-toggle="collapse"
                            data-target="#navbarNav" aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
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
                                <UnauthenticatedHome login_callback={login}/>
                            </Route>
                        </Switch>
                    </div>
                </Router>
            );
        }
        return (
            <Router>
                <div className="col-sm-10">
                    <nav className="navbar navbar-expand-lg navbar-light bg-light justify-content-between">
                        <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
                            <span className="navbar-toggler-icon"></span>
                        </button>
                        <div className="collapse navbar-collapse" id="navbarNav">
                            <ul className="navbar-nav">
                                <li className="nav-item">
                                    <Link className="nav-link" to="/">Home</Link>
                                </li>
                                <li className="nav-item">
                                    <Link className="nav-link" to="/storage">Storage</Link>
                                </li>
                                <li className="nav-item">
                                    <Link className="nav-link" to="/about">About</Link>
                                </li>
                            </ul>
                            <ul className="navbar-nav ml-auto">
                                <li className="nav-item">
                                    <button className="btn btn-default" onClick={logout}>
                                        Log Out {user.email}
                                    </button>
                                </li>
                            </ul>
                        </div>
                    </nav>
                </div>
                <Switch>
                    <Route path="/storage">
                        <Storage className="col-sm-10"/>
                    </Route>
                    <Route path="/about">
                        <About className="col-sm-10" />
                    </Route>
                    <Route path="/">
                        <User className="col-sm-10" email={user.email} logout={logout}/>
                    </Route>
                </Switch>
            </Router>
        );
    }

    return render();
}