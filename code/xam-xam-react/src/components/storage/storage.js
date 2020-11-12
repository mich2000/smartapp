import React from 'react';
import {InputStorageDialog} from '../dialogs/InputStorage';

export function Storage() {
    return (
        <div>
            <h2>Storages</h2>
            <p>
                <InputStorageDialog/>
            </p>
        </div>
    );
}