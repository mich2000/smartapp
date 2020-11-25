import React,{useState} from 'react';

function extract_storage_name() {
    let search = window.location.search;
    let params = new URLSearchParams(search);
    return params.get('storage');
}

export const Product = (props) => {
    const [products, setProducts] = useState([]);
    const storage = extract_storage_name();

    function add_product_list(product_list) {
        setProducts(product_list);
    }

    return (
        <div className="col-sm-10">
            <h2>Products from {storage}</h2>
        </div>
    );
}