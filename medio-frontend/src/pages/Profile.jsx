import { Navigate } from "react-router-dom";
import { useAuth } from "../provider/AuthProvider";
import Layout from "./Layout";
import { useEffect, useState } from "react";
import userService from "../services/users";
import articleService from "../services/articles";
import ProfileSideBar from "../components/ProfileSideBar";
import Publish from "../components/Publish";
import ArticleCard from "../components/ArticleCard";
import ProfileArticleCard from "../components/ProfileArticleCard";

const initial = "Published Articles";

const Profile = () => {
    const { userid, token } = useAuth();
    const [articles, setArticles] = useState([]);
    const [selectedTab, setSelectedTab] = useState(initial);

    const handleSelectedTab = (tab) => {
        setSelectedTab(tab);
    }

    if (!token) {
        return <Navigate to="/login" />
    }

    useEffect(() => {
        const fetchUserArticles = async () => {
            try {
                console.log("TOKEN: " + token);
                const response = await userService.getAllForUser(token, userid);
                //const response = await articleService.getAll();
                console.log(response);
                setArticles(response);
            } catch (e) {
                console.error(e);
            }
        }

        fetchUserArticles();
    }, []);

    return (
        <Layout>
            <div className="flex min-h-screen">
                <div className="my-10 shadow-lg">
                    <ProfileSideBar onSelectedTab={handleSelectedTab} initialTab={initial} />
                </div>
                <div className="flex flex-col items-center flex-1">
                    {selectedTab && (
                        <>
                            {selectedTab === "Published Articles" &&
                                <>
                                    <div className="font-bold text-3xl mt-10 mx-10">Published Articles</div>
                                    <div className="grid grid-cols-1 gap-12 md:grid-cols-2 xl:grid-cols-2 m-10">
                                        {articles?.map(a => (
                                            <ArticleCard id={a.id} title={a.title} content={a.content} key={a.id} />
                                        ))}
                                    </div>
                                </>}
                            {selectedTab === "Publish New" && <Publish token={token} id={userid} />}
                        </>
                    )}
                </div>

            </div>
        </Layout>
    )
}

export default Profile;