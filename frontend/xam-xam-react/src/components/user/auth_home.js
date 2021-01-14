import React from 'react';
import { UserInfo } from './user_info';
import { Link } from "react-router-dom";
import {OldestProduct} from './oldest_product';

export function AuthHome() {
    return <div className="center-info-user col-sm-10">
            <UserInfo />
            <Link className="btn btn-primary" to="/storage">Go to storages</Link>
            <OldestProduct/>
        </div>;
}