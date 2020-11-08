import React,{useContext, useEffect} from 'react';
import api_functions from '../api';
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";
import UnauthenticatedHome, {About} from './normal_home';
import {User} from './user/user';
import {AppContext} from '../state';

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
        if(!user.loggedIn) {
            fetch(api_functions.get_api() + "/auth/validate",api_functions.method_get())
            .then((api_call) => {
                if(api_call.status === 200) {
                    setUser({email : user.email, loggedIn: true})
                } else {
                    console.log(api_call.body)
                }
            })
            .catch((e) => console.error(`Could not send through the request. error: ${e}`));
        }
    })

    function render() {
        if(!user.loggedIn) {
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
                    <div className="div-inline-block">
                        <button className="btn btn-primary" onClick={logout}>
                            Log Out
                        </button>
                    </div>
                </div>
                <Switch>
                    <Route path="/about">
                        <About />
                    </Route>
                    <Route path="/">
                        <User email={user.email} logout={logout}/>
                    </Route>
                </Switch>
            </Router>
        );
    }

    return render();
}