import React,{useState} from 'react';
import {InputStorageDialog} from './input_storage';
import {Storages} from './storages';

export const Storage = () => {
    const [storages, setStorages] = useState([]);

    function add_storage_list(storage_list) {
        setStorages(storage_list);
    }

    function add_storage(storage) {
        setStorages(storages.concat([[storage.name, storage.kind]]));
    }

    function edit_storage(edited_storage) {
        setStorages(storages.filter(storage => storage[0] !== edited_storage.storage_name).concat([[edited_storage.new_storage_name || edited_storage.storage_name, edited_storage.new_kind]]));
    }

    function remove_storage(storage_name) {
        setStorages(storages.filter(storage => storage[0] !== storage_name));
    }

    return (
        <div className="col-sm-10">
            <h2>Storages</h2>
            <InputStorageDialog set_storage_list={(e) => add_storage_list(e) } add_storage={(e) => add_storage(e) }/>
            <Storages storages={storages} remove_storage={(e) => remove_storage(e)} edit_storage={(e) => edit_storage(e)}/>
        </div>
    );
}