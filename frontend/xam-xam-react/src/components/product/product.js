import React,{useState,useEffect} from 'react';
import api_functions from '../../api';
import {Products} from './products';

function extract_storage_name() {
    let search = window.location.search;
    let params = new URLSearchParams(search);
    return params.get('storage');
}

export const Product = (props) => {
    const [products, setProducts] = useState([]);
    const storage = extract_storage_name();

    useEffect(() => {
        fetch(api_functions.get_business_api() + '/products/' + storage, api_functions.method_get())
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.json()
                .then((json) => setProducts(json.products))
                .catch((e) => {
                    console.error(`Could not send through the request. error: ${e}`);
                });
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    },[])

    return (
        <div className="col-sm-10">
            <h2>Products from {storage}</h2>
            <Products products={products}/>
        </div>
    );
}