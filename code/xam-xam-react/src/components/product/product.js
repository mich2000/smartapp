import React,{useState} from 'react';

export const Product = (props) => {
    const [products, setProducts] = useState([]);
    const storage = props.match.params.storage;

    function add_product_list(product_list) {
        setProducts(product_list);
    }

    return (
        <div className="col-sm-10">
            <h2>Products from {storage}</h2>
        </div>
    );
}