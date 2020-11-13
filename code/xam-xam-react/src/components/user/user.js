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
        <>
            {
                (message.msg !== '' && message.error)? <span className="font-weight-bold text-danger">{message.msg}</span> : <span className="font-weight-bold text-success">{message.msg}</span>
            }
            <div id="accordion" className="col-sm-10 mt-5">
                <div className="card text-center">
                    <div className="card-header" id="UserInfo">
                        <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserInfo"
                        aria-expanded="true" aria-controls="collapseUserInfo">User statistics</button>
                    </div>
                    <div id="collapseUserInfo" className="collapse" aria-labelledby="UserInfo" data-parent="#accordion">
                        <div className="card-body">
                            <UserInfo changeEmail={email => changeEmail(email)}/>
                        </div>
                    </div>
                </div>
                <div className="card text-center">
                    <div className="card-header" id="UpdateEmail">
                        <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUpdateEmail"
                        aria-expanded="true" aria-controls="collapseUpdateEmail">Update email</button>
                    </div>
                    <div id="collapseUpdateEmail" className="collapse" aria-labelledby="UpdateEmail" data-parent="#accordion">
                        <div className="card-body">
                            <ChangeEmail changeEmail={email => changeEmail(email)} message_callback={(e,s) => set_message(e,s)}/>
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
                            <ChangePwd message_callback={(e,s) => set_message(e,s)} email={user.email}/>
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
                            <DeleteProfile message_callback={(e,s) => set_message(e,s)} email={user.email} logout={props.logout}/>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}