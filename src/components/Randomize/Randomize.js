import React from 'react';

import { Redirect } from 'react-router-dom';

export class Randomize extends React.Component {
    constructor(props, context) {
        super(props, context);

        this.state = {
            error: null,
            isLoaded: false,
            playlist: '',
            loginState: localStorage.getItem("logged-in")
        };
    }

    componentDidMount() {
        fetch("https://playlist-randomizer-api.herokuapp.com/randomize")
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
        const { error, isLoaded, playlist, loginState } = this.state;
        if (loginState) {
            return <Redirect to="/login"/>
        }
        if (error) {
            return <div>Error: {error.message}</div>;
        } else if (!isLoaded) {
            return <div>Loading...</div>;
        } else {
            return (
                <a href={playlist.href}>{playlist.name}</a>
            );
        }
    }
}

export default Randomize
