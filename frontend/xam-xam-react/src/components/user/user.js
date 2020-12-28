import React, { useContext }  from 'react';
import {ChangeEmail} from './change_email';
import {ChangePwd} from './change_pwd';
import {DeleteProfile} from './delete_profile';
import {AppContext} from '../../state';

export function User(props) {
    const [user,setUser] = useContext(AppContext);
    
    function changeEmail(new_email) {
        setUser({email : new_email,loggedIn : true});
    }

    return (
        <>
            <div id="accordion" className="col-sm-10 mt-5">
                <div className="card text-center">
                    <div className="card-header" id="UpdateEmail">
                        <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUpdateEmail"
                        aria-expanded="true" aria-controls="collapseUpdateEmail">Update email</button>
                    </div>
                    <div id="collapseUpdateEmail" className="collapse" aria-labelledby="UpdateEmail" data-parent="#accordion">
                        <div className="card-body">
                            <ChangeEmail changeEmail={email => changeEmail(email)}/>
                        </div>
                    </div>
                </div>
                <div className="card text-center">
                    <div className="card-header" id="ChangePwd">
                        <button className="btn btn-link" data-toggle="collapse" data-target="#collapseChangePwd"
                        aria-expanded="true" aria-controls="collapseChangePwd">Change password</button>
                    </div>
                    <div id="collapseChangePwd" className="collapse" aria-labelledby="ChangePwd" data-parent="#accordion">
                        <div className="card-body">
                            <ChangePwd email={user.email}/>
                        </div>
                    </div>
                </div>
                <div className="card text-center">
                    <div className="card-header" id="DeleteProfile">
                        <button className="btn btn-link" data-toggle="collapse" data-target="#collapseDeleteProfile"
                        aria-expanded="true" aria-controls="collapseDeleteProfile">Delete your profile</button>
                    </div>
                    <div id="collapseDeleteProfile" className="collapse" aria-labelledby="ChangePwd" data-parent="#accordion">
                        <div className="card-body">
                            <DeleteProfile email={user.email} logout={props.logout}/>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}