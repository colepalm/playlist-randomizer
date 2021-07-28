import React from 'react';
// import PropTypes from 'prop-types';
// import styles from './Randomize.module.css';

export class Randomize extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            error: null,
            isLoaded: false,
            playlist: ''
        };
    }

    componentDidMount() {
        fetch("http://localhost:3000/randomize")
            .then(res => res.json())
            .then(
                (result) => {
                    this.setState({
                        isLoaded: true,
                        playlist: result.playlist
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
        const { error, isLoaded, playlist } = this.state;
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
