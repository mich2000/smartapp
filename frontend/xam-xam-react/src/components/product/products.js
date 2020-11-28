import React from 'react';

const ProductUnit = (item_info) => {
    let difference_in_time = new Date(item_info[3]) - new Date();
    let difference_in_days = Math.floor(difference_in_time / (1000 * 3600 * 24));

    return (
        <tr>
            <th>
                {item_info[1]}
            </th>
            <th>
                {item_info[2]}
            </th>
            <th>
                {difference_in_days}
            </th>
            <th>
                {item_info[4]}
            </th>
            <th>
                <input hidden value={item_info[0]} name="product_id"/>
            </th>
        </tr>
    );
}

export const Products = (props) => {
    return (
        <>
            { props.products.length !== 0 && 
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
                            return ( <ProductUnit {...item}/> );
                        })}
                    </tbody>
                </table>
            </> ||
            <h2>No products in {props.storage}</h2>}
        </>
    );
}