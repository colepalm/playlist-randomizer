import React from 'react';
import { Redirect } from 'react-router-dom';

import { BASE_URL } from "../../global/baseUrl";

export class Randomize extends React.Component {
    constructor(props, context) {
        super(props, context);

        fetch(`${BASE_URL}/check`)
            .then(res =>
                res.status === 401 ? this.state.loggedIn = false : this.state.loggedIn = true
            )

        this.state = {
            error: null,
            isLoaded: false,
            playlist: '',
            loggedIn: true
        };
    }



    componentDidMount() {
        fetch(`${BASE_URL}/randomize`)
            .then(res => res.json())
            .then(
                (result) => {
                    this.setState({
                        isLoaded: true,
                        playlist: result
                    });
                },
                (error) => {
                    this.setState({
                        isLoaded: true,
                        error
                    });
                }
            )
    }

    render() {
        const { error, isLoaded, playlist, loggedIn } = this.state;
        if (!loggedIn) {
            return <Redirect to="/login"/>
        }
        if (error) {
            return <div>Error: {error.message}</div>;
        } else if (!isLoaded) {
            return <div>Loading...</div>;
        } else {
            return (
                <p>{playlist.name}</p>
            );
        }
    }
}

export default Randomize
