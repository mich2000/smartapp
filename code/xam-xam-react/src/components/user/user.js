import React, { useState, useContext }  from 'react';
import {UserInfo} from './user_info';
import {ChangeEmail} from './change_email';
import {ChangePwd} from './change_pwd';
import {DeleteProfile} from './delete_profile';
import {AppContext} from '../../state';

export function User(props) {
    const [message,setMessage] = useState({msg : '', error : false});
    const [user,setUser] = useContext(AppContext);

    function set_message(msg,error) {
        setMessage({msg:msg,error:error})
    }
    
    function changeEmail(new_email) {
        setUser({email : new_email,loggedIn : true});
    }

    return (
        <div>
            <h2>{user.email}</h2>
            {
                (message.msg !== '' && message.error)? <span className="font-weight-bold text-danger">{message.msg}</span> : <span className="font-weight-bold text-success">{message.msg}</span>
            }
            <div id="accordion" className="col-sm-8 m-3">
                <div className="card">
                    <div className="card-header" id="UserInfo">
                        <h4 className="mb-0">
                            <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserInfo" aria-expanded="true" aria-controls="collapseUserInfo">User statistics</button>
                        </h4>
                    </div>
                    <div id="collapseUserInfo" className="collapse" aria-labelledby="UserInfo" data-parent="#accordion">
                        <div className="card-body">
                            <UserInfo changeEmail={email => changeEmail(email)}/>
                        </div>
                    </div>
                </div>
                <div className="card">
                    <div className="card-header" id="UpdateEmail">
                        <h4 className="mb-0">
                            <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUpdateEmail" aria-expanded="true" aria-controls="collapseUpdateEmail">Update email</button>
                        </h4>
                    </div>
                    <div id="collapseUpdateEmail" className="collapse" aria-labelledby="UpdateEmail" data-parent="#accordion">
                        <div className="card-body">
                            <ChangeEmail changeEmail={email => changeEmail(email)} message_callback={(e,s) => set_message(e,s)}/>
                        </div>
                    </div>
                </div>
                <div className="card">
                    <div className="card-header" id="ChangePwd">
                        <h4 className="mb-0">
                            <button className="btn btn-link" data-toggle="collapse" data-target="#collapseChangePwd" aria-expanded="true" aria-controls="collapseChangePwd">Change password</button>
                        </h4>
                    </div>
                    <div id="collapseChangePwd" className="collapse" aria-labelledby="ChangePwd" data-parent="#accordion">
                        <div className="card-body">
                            <ChangePwd message_callback={(e,s) => set_message(e,s)} email={user.email}/>
                        </div>
                    </div>
                </div>
                <div className="card">
                    <div className="card-header" id="DeleteProfile">
                        <h4 className="mb-0">
                            <button className="btn btn-link" data-toggle="collapse" data-target="#collapseDeleteProfile" aria-expanded="true" aria-controls="collapseDeleteProfile">Delete your profile</button>
                        </h4>
                    </div>
                    <div id="collapseDeleteProfile" className="collapse" aria-labelledby="ChangePwd" data-parent="#accordion">
                        <div className="card-body">
                            <DeleteProfile message_callback={(e,s) => set_message(e,s)} email={user.email} logout={props.logout}/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}