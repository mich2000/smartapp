import React,{useContext, useEffect} from 'react';
import api_functions from '../api';
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";
import UnauthenticatedHome, {About} from './normal_home';
import {User} from './user/user';
import {AppContext} from '../state';
import {Storage} from './storage/storage';
import {UserInfoPopup} from './user/user_info_popup';
import {Product} from './product/product';
import {AuthHome} from './user/auth_home';

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
                api_call.text()
                .then(text => {
                    if(text == "authenticated") {
                        setUser({email : user.email, loggedIn: true});
                    }
                })
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
    },[user.email,setUser])

    return (
        !user.loggedIn? <Router>
            <div className="col-sm-10">
                <nav className="navbar navbar-expand-lg navbar-light bg-light">
                    <button className="navbar-toggler" type="button" data-toggle="collapse"
                    data-target="#navbarNav" aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
                        <span className="navbar-toggler-icon"></span>
                    </button>
                    <div className="collapse navbar-collapse" id="navbarNav">
                        <ul className="navbar-nav">
                            <Link className="nav-link" to="/">Home</Link>
                            <Link className="nav-link" to="/about">About</Link>
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
        </Router> : <Router>
            <div className="col-sm-10">
                <nav className="navbar navbar-expand-lg navbar-light bg-light justify-content-between">
                    <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNav" 
                    aria-controls="navbarNavDropdown" aria-expanded="false" aria-label="Toggle navigation">
                        <span className="navbar-toggler-icon"></span>
                    </button>
                    <div className="collapse navbar-collapse" id="navbarNav">
                        <ul className="navbar-nav">
                            <Link className="nav-link" to="/">Home</Link>
                            <Link className="nav-link" to="/storage">Storage</Link>
                            <Link className="nav-link" to="/about">About</Link>
                        </ul>
                        <ul className="navbar-nav ml-auto">
                            <UserInfoPopup trigger={<a className="nav-link">
                            <svg width="1em" height="1em" viewBox="0 0 16 16" className="bi bi-info-circle-fill svg-info" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                                <path fillRule="evenodd" d="M8 16A8 8 0 1 0 8 0a8 8 0 0 0 0 16zm.93-9.412l-2.29.287-.082.38.45.083c.294.07.352.176.288.469l-.738 3.468c-.194.897.105 1.319.808 1.319.545 0 1.178-.252 1.465-.598l.088-.416c-.2.176-.492.246-.686.246-.275 0-.375-.193-.304-.533L8.93 6.588zM8 5.5a1 1 0 1 0 0-2 1 1 0 0 0 0 2z"/>
                            </svg>
                            </a>}/>
                            <li className="nav-item">
                                <div className="dropdown">
                                    <button className="btn btn-secondary dropdown-toggle nav-link" type="button" id="dropDownProfile" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                                        <svg width="1em" height="1em" viewBox="0 0 16 16" className="bi bi-file-person-fill svg-info" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                                            <path fillRule="evenodd" d="M12 0H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V2a2 2 0 0 0-2-2zm-1 7a3 3 0 1 1-6 0 3 3 0 0 1 6 0zm-3 4c2.623 0 4.146.826 5 1.755V14a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-1.245C3.854 11.825 5.377 11 8 11z"/>
                                        </svg>
                                    </button>
                                    <div className="dropdown-menu" aria-labelledby="dropDownProfile">
                                        <Link className="dropdown-item" to="/profile">{user.email}</Link>
                                        <Link className="dropdown-item" to="#" onClick={logout}>Log out</Link>
                                    </div>
                                </div>
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
                <Route path="/products">
                    <Product/>
                </Route>
                <Route path="/profile">
                    <User className="col-sm-10" email={user.email} logout={logout}/>
                </Route>
                <Route path="/">
                    <AuthHome/>
                </Route>
            </Switch>
        </Router>
    );
}