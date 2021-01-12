import React,{useState,useEffect} from 'react';
import api_functions from '../../api';
import {Products} from './products';
import {InputProductDialog} from './input_product';

function extract_storage_name() {
    let search = window.location.search;
    let params = new URLSearchParams(search);
    return params.get('storage');
}

export const Product = () => {
    const [products, setProducts] = useState([]);
    const storage = extract_storage_name();

    function add_product(product) {
        setProducts(products.concat([[product[0],product[1], product[2],product[3],product[4]]]));
    }

    function edit_product(product) {
        setProducts(products.map(prod => (parseInt(prod[0]) !== parseInt(product.id)) ? prod : [product.id,product.name,parseInt(product.amount), product.date,product.kind]));
    }

    function remove_product(product_id) {
        setProducts(products.filter(product => parseInt(product[0]) !== parseInt(product_id)));
    }

    useEffect(() => {
        fetch(api_functions.get_business_api() + '/products/' + storage, api_functions.method_get())
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.json()
                .then((json) => {
                    setProducts(json.products);
                })
                .catch((e) => {
                    console.error(`Could not send through the request. error: ${e}`);
                });
            }
        }).catch((e) => {
            console.error(`Could not send through the request. error: ${e}`);
        });
    },[storage])

    return (
        <div className="col-sm-10">
            <InputProductDialog storage={storage} add_product={(e) => add_product(e)}/>
            <Products products={products} storage={storage} remove_product={(e) => remove_product(e)} edit_product={(e) => edit_product(e)}/>
        </div>
    );
}