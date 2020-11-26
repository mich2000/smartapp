import React from 'react';
import Popup from 'reactjs-popup';

export function DeleteStoragePopup(props) {
    return (
        <Popup trigger={<button className="badge badge-pill badge-danger m-2" value={props.item} type="button">X</button>}
        modal nested>
            {
                close =>
                <div className="modal-dialog">
                    <div className="modal-header">Are you sure to remove a storage?</div>
                    <div className="modal-content">
                        <button className="btn btn-primary modal-input" value={props.item[0]}
                        onClick={(e) => {
                            props.delete_storage(e);
                            close();}}>
                            Remove {props.item[0]}
                        </button>
                    </div>
                </div>
            }
        </Popup>
    );
}