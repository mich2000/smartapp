import React,{useState,useEffect} from 'react';
import {InputStorageDialog} from './input_storage';
import {Storages} from './storages';
import api_functions from '../../api';
import {showError} from '../../toast';

export const Storage = () => {
    const [storages, setStorages] = useState([]);

    useEffect(() => {
        fetch(api_functions.get_business_api() + '/storages', api_functions.method_get())
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.json()
                .then((json) => setStorages(json.storages))
                .catch((e) => {
                    console.error(`Could not send through the request. error: ${e}`);
                });
            } else {
                api_call.text()
                .then(err => showError(err));
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    },[])

    function add_storage(storage) {
        setStorages(storages.concat([[storage.name, storage.kind]]));
    }

    function edit_storage(edited_storage) {
        setStorages(storages.map(stor => (stor[0] !== edited_storage.storage_name) ? stor : [edited_storage.new_storage_name || edited_storage.storage_name, edited_storage.new_kind]));
    }

    function remove_storage(storage_name) {
        setStorages(storages.filter(storage => storage[0] !== storage_name));
    }

    return <div className="col-sm-10">
            <h2>Storages</h2>
            <InputStorageDialog add_storage={(e) => add_storage(e) }/>
            <Storages storages={storages} remove_storage={(e) => remove_storage(e)} edit_storage={(e) => edit_storage(e)}/>
        </div>;
}