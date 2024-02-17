import { Card, Button } from "flowbite-react"
import { Link } from "react-router-dom";
import { TiDelete } from "react-icons/ti";

const ProfileArticleCard = (props) => {
    return (
        <Card
            className="max-w-sm"
            imgSrc="./assets/image-1.jpg" horizontal
        >
            <div className="flex justify-between">
                <Link to={`/article/${props.id}`} className="mr-5">
                    <h5 className="text-2xl font-bold tracking-tight text-gray-900 dark:text-white">
                        {props.title}
                    </h5>
                </Link>
                <Button pill color="failure" className="text-sm">
                    <TiDelete className="mr-1 h-5 w-5" />
                    Remove
                </Button>

            </div>
            <p className="text-sm font-normal text-gray-700 dark:text-gray-400">
                {props.content.split(' ').slice(0, 20).join(' ')}...
                <Link to={`/article/${props.id}`} className="text-teal-700 font-medium">Read more</Link>
            </p>
        </Card>
    )
}

export default ProfileArticleCard;