import { useState } from "react"
import { Sidebar } from "flowbite-react";
import { FaList } from "react-icons/fa";
import { MdCreate } from "react-icons/md";

const ProfileSideBar = ({ onSelectedTab, initialTab }) => {
    const menuTabs = ['Published Articles', 'Publish New'];
    const [selectedTab, setSelectedTab] = useState(initialTab);
    
    const handleTabClick = (item) => {
        setSelectedTab(item);
        onSelectedTab(item);
    }
    return (
        <Sidebar aria-label="Profile sidebar">
            <Sidebar.Items>
                <Sidebar.ItemGroup>
                    {menuTabs.map((tab, index) => (
                        <Sidebar.Item key={index} 
                            onClick={() => handleTabClick(tab)}
                            className="hover:cursor-pointer"
                            icon={index === 0 ? FaList : MdCreate}>
                            {tab}
                        </Sidebar.Item>
                    ))}
                </Sidebar.ItemGroup>
            </Sidebar.Items>
        </Sidebar>
    )
}

export default ProfileSideBar;