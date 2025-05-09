import { Link } from "react-router-dom"
import "./NavBar.css"

export const NavBar = () => {
    return (
        <div className="navbar_container">
            <p> <Link to="/" id="navbar_container_title">ReMusic</Link> </p>
            <Link to="/upload">     New Upload      </Link>
            <Link to="/gallery">    Gallery         </Link>
        </div>
    )
}