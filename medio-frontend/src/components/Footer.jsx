import { Footer as Footerfb } from "flowbite-react"

const Footer = () => {
    return (
        <Footerfb container className="py-10">
            <Footerfb.Copyright href="#" by="Medio™" year={2024} />
            <Footerfb.LinkGroup>
                <Footerfb.Link href="#">About</Footerfb.Link>
                <Footerfb.Link href="#">Privacy Policy</Footerfb.Link>
                <Footerfb.Link href="#">Licensing</Footerfb.Link>
                <Footerfb.Link href="#">Contact</Footerfb.Link>
            </Footerfb.LinkGroup>
        </Footerfb>
    )
}

export default Footer;