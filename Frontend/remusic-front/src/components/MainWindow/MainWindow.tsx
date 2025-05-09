import { Outlet } from "react-router-dom"
import { Footer } from "../Footer/Footer"
import { HeroBanner } from "../HeroBanner/HeroBanner"
import { NavBar } from "../NavBar/NavBar"
import "./MainWindow.css"

export const MainWindow = () => {
    return (
        <div className="mainview_container">
            <div>
                <NavBar />
                <HeroBanner />
            </div>
                <Outlet />
            <div>
                <Footer />
            </div>
        </div>
    )
}