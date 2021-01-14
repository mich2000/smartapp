import Popup from 'reactjs-popup';
import {UserInfo} from './user_info';
import React from 'react';

export function UserInfoPopup(props) {
    return <Popup trigger={props.trigger} modal nested>
            {
                <div className="modal-dialog">
                    <div className="modal-header d-flex justify-content-center">Overview</div>
                    <div className="modal-content">
                        <UserInfo/>
                    </div>
                </div>
            }
        </Popup>;
}