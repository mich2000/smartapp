import React, { useState }  from 'react';
import {UserInfo} from './user_info';

export function User(props) {
    const [email, SetEmail] = useState(props.email);

    return (
        <div>
            <h2>{email}</h2>
            <div id="accordion" className="col-sm-8 m-3">
                <div className="card">
                    <div className="card-header" id="UserInfo">
                        <h4 className="mb-0">
                            <button className="btn btn-link" data-toggle="collapse" data-target="#collapseUserInfo" aria-expanded="true" aria-controls="collapseUserInfo">User statistics</button>
                        </h4>
                    </div>
                    <div id="collapseUserInfo" className="collapse" aria-labelledby="UserInfo" data-parent="#accordion">
                        <div className="card-body">
                            <UserInfo/>
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
                            Unimplemented
                        </div>
                    </div>
                </div>
                <div className="card">
                    <div className="card-header" id="ChangePwd">
                        <h4 className="mb-0">
                            <button className="btn btn-link" data-toggle="collapse" data-target="#collapseChangePwd" aria-expanded="true" aria-controls="collapseChangePwd">Update password</button>
                        </h4>
                    </div>
                    <div id="collapseChangePwd" className="collapse" aria-labelledby="ChangePwd" data-parent="#accordion">
                        <div className="card-body">
                            Unimplemented
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}