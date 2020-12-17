import React, {useEffect, useState} from 'react';
import { Link } from "react-router-dom";
import api_functions from '../../api';

const ProductUnit = (props) => {
    return (
        <tr>
            <th>
                {props.item_info.name}
            </th>
            <th>
                {props.item_info.amount}
            </th>
            <th>
                {props.item_info.date}
            </th>
            <th>
                {props.item_info.kind}
            </th>
            <th>
                <Link className="text-dark" to={{
                    pathname: "/products",
                    search : "?storage=" + props.item_info.storage_name
                }}>{props.item_info.storage_name}</Link>
            </th>
        </tr>
    );
}

export function OldestProduct() {
    const [oldestProducts,setOldestProducts] = useState([]);
    
    useEffect(() => {
        fetch(api_functions.get_api() + '/user/recent/products', api_functions.method_get())
        .then(request => request.json())
        .then(body_json => {
            console.log(body_json.units);
            setOldestProducts(body_json.units);
        })
        .catch((e) => console.error(`Could not send through the request. error: ${e}`));
    },[])

    return (
        <>
            {( oldestProducts.length !== 0)?
            <>
                <h2>5 oldest products</h2>
                <table className="table">
                    <thead>
                        <tr>
                            <th>name</th>
                            <th>#</th>
                            <th>Date</th>
                            <th>Kind</th>
                            <th>Go to storage</th>
                        </tr>
                    </thead>
                    <tbody>
                        {oldestProducts.map((item, i) => {
                            return ( <ProductUnit key={i} item_info={item}/> );
                        })}
                    </tbody>
                </table>
            </> : <h2>No products</h2>}
        </>
    );
}