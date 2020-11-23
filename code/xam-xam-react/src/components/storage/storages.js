import React from 'react';
import api_functions from '../../api';
import {DeleteStoragePopup} from './delete_storage';
import {EditStoragePopup} from './edit_storage';

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

    function edit_storage(edited_storage) {
        let options = api_functions.method_put();
        options.body = JSON.stringify({
            storage_name: edited_storage.storage_name,
            new_storage_name : edited_storage.new_storage_name || null,
            new_kind: edited_storage.new_kind
        });
        fetch(api_functions.get_business_api() + '/storage',options)
        .then((api_call) => {
            if(api_call.status === 200) {
                props.edit_storage({
                    storage_name: edited_storage.storage_name,
                    new_storage_name : edited_storage.new_storage_name || null,
                    new_kind: edited_storage.new_kind
                });
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    }

    return (
        <>
            {props.storages.length !== 0 && 
            <ul className="list-style-none d-flex flex-wrap">
                {props.storages.map((item, i) => {
                    return (
                        <li className="tag-li-user badge badge-pill badge-info big-text m-1" key={i}>
                            {item[0]} - {item[1]}
                            <EditStoragePopup edit_storage={(e) => edit_storage(e)} item={item}/>
                            <DeleteStoragePopup delete_storage={(e) => delete_storage(e)} item={item}/>
                        </li>
                    );
                })}
            </ul>}
        </>
    );
}