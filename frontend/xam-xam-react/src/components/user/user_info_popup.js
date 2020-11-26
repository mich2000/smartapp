import Popup from 'reactjs-popup';
import {UserInfo} from './user_info';
import React from 'react';

export function UserInfoPopup() {
    return (
        <Popup trigger={<button className="btn btn-default">Overview</button>} modal nested>
            {
                <div className="modal-dialog">
                    <div className="modal-header">Overview</div>
                    <div className="modal-content">
                        <UserInfo/>
                    </div>
                </div>
            }
        </Popup>
    );
}