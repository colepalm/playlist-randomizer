import React from 'react';

export class LoggedIn extends React.Component {
    constructor(props) {
        super(props);

        localStorage.setItem("logged-in", "true")

        this.props.history.push("/randomize")
    }
}
