import React from 'react';
import {DeleteProductPopup} from './delete_product';
import {EditProductDialog} from './edit_product';
import api_functions from '../../api';
import {showError} from '../../toast';

const ProductUnit = (props) => {
    let item_info = [...props.item_info];
    let difference_in_time = new Date(item_info[3]) - new Date();
    let difference_in_days = Math.ceil(difference_in_time / (1000 * 3600 * 24));

    return (
        <tr>
            <th>
                {item_info[1]}
            </th>
            <th>
                {item_info[2]}
            </th>
            <th>
                {difference_in_days} days left
            </th>
            <th>
                {item_info[4]}
            </th>
            <th>
                <EditProductDialog item_info={item_info} storage={props.storage} edit_product={(e) => props.edit_product(e)}/>
                <DeleteProductPopup item={item_info} delete_product={(id) => props.delete_product(id)}/>
            </th>
        </tr>
    );
}

export const Products = (props) => {
    function delete_product(id) {
        let options = api_functions.method_delete();
        options.body = JSON.stringify({
            id : parseInt(id),
            storage_name : props.storage
        });
        fetch(api_functions.get_business_api() + '/product',options)
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.text()
                .then(text => {
                    if(text !== 'No internet connection') {
                        props.remove_product(id);
                    } else {
                        showError(text);
                    }
                });
            } else {
                api_call.text()
                .then(err => showError(err));
            }
        }).catch(() => showError('No internet connection'));
    }

    function edit_product(product) {
        let options = api_functions.method_put();
        options.body = JSON.stringify({
            id : product.id,
            storage_name: props.storage,
            name: product.name,
            amount: product.amount,
            peremption_date: product.date,
            kind: product.kind
        });
        fetch(api_functions.get_business_api() + '/product', options)
        .then((api_call) => {
            if(api_call.status === 200) {
                api_call.text()
                .then(text => {
                    if(text !== 'No internet connection') {
                        props.edit_product({
                            id : product.id,
                            name : product.name,
                            amount : product.amount,
                            date : product.date,
                            kind : product.kind
                        });
                    } else {
                        showError(text);
                    }
                });
            } else {
                api_call.text()
                .then(err => showError(err));
            }
        }).catch(() => showError('No internet connection'));
    }

    return <>
            {( props.products.length !== 0)?
            <>
                <h2>Products from {props.Datestorage}</h2>
                <table className="table">
                    <thead>
                        <tr>
                            <th>name</th>
                            <th>#</th>
                            <th>days until</th>
                            <th>Kind</th>
                            <th></th>
                        </tr>
                    </thead>
                    <tbody>
                        {props.products.map((item, i) => {
                            return ( <ProductUnit key={i} item_info={item} delete_product={(id) => delete_product(id)} storage={props.storage} edit_product={(e) => edit_product(e)}/> );
                        })}
                    </tbody>
                </table>
            </> : <h2>No products in {props.storage}</h2>}
        </>;
}