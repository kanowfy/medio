import { Button, Navbar } from "flowbite-react";
import { Link } from "react-router-dom";
import { useAuth } from "../provider/AuthProvider";

// we would need some state to make the login button appear/disappear
const Header = () => {
    const { userid } = useAuth();
    return (
        <Navbar fluid rounded className="py-5 shadow-md">
            <Link to="/" className="flex flex-row ml-16">
                <img src="assets/medio.svg" className="mr-3 h-6 sm:h-9" alt="Medio" />
                <span className="self-center whitespace-nowrap text-3xl font-black dark:text-white tracking-widest font-mono">MEDIO</span>
            </Link>
            <Navbar.Toggle />
            <Navbar.Collapse className="mr-16">
                {userid ?
                    <>
                        <Navbar.Link href="/profile">Profile</Navbar.Link>
                        <Navbar.Link href="/logout">Logout</Navbar.Link>
                    </> : <>
                        <Navbar.Link href="/login">
                            <Button outline gradientDuoTone="greenToBlue">Login</Button>
                        </Navbar.Link>
                        <Navbar.Link href="/register">
                            <Button gradientDuoTone="greenToBlue">Register</Button>
                        </Navbar.Link>
                    </>
                }
            </Navbar.Collapse>
        </Navbar>
    )
}

export default Header;