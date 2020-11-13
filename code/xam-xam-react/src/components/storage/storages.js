import React from 'react';
import api_functions from '../../api';
import Popup from 'reactjs-popup';

export const Storages = (props) => {
    function delete_storage(event) {
        let name_storage = event.target.value;
        event.preventDefault();
        event.stopPropagation();
        let options = api_functions.method_delete();
        options.body = JSON.stringify({
            name : name_storage
        });
        fetch(api_functions.get_business_api() + '/storage',options)
        .then((api_call) => {
            if(api_call.status === 200) {
                props.remove_storage(name_storage);
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    }
    return (
        <>
            {props.storages.length !== 0 && 
            <ul className="list-style-none d-flex flex-wrap mb-n2">
                {props.storages.map((item, i) => {
                    return (
                        <li className="mb-2 mr-2 tag-li-user badge badge-pill badge-info big-text" key={i}>
                            {item[0]} - {item[1]}
                            <Popup trigger={<button className="badge badge-pill badge-danger m-2" value={item} type="button">X</button>}
                            modal nested>
                                {
                                    close =>
                                    <div className="modal-dialog">
                                        <div className="modal-header">Are you sure to remove a storage? </div>
                                        <div className="modal-content">
                                            <button className="btn btn-primary modal-input" value={item[0]}
                                            onClick={(e) => {
                                                delete_storage(e);
                                                close();}}>
                                                Remove {item[0]}
                                            </button>
                                        </div>
                                    </div>
                                }
                            </Popup>
                        </li>
                    );
                })}
            </ul>}
        </>
    );
}