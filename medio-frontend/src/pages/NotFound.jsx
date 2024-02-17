const NotFound = () => {
    return (
        <div className="container z-10 flex items-center justify-between h-screen px-6 pt-32 mx-auto md:pt-0">
        <div className="container relative flex flex-col-reverse items-center justify-between px-6 mx-auto lg:flex-row">
            <div className="w-full mb-16 text-center md:mb-8 lg:text-left">
                <h1 className="mt-12 font-sans text-5xl font-light text-center text-gray-700 lg:text-left lg:text-8xl md:mt-0">
                    Sorry, this page isn&#x27;t available
                </h1>
                <a href="/" className="px-2 py-2 mt-36 text-lg font-light transition duration-200 ease-in bg-yellow-300 border-2 border-gray-700 w-36 hover:bg-yellow-400 focus:outline-none">
                    Go back home
                </a>
            </div>
            <div className="relative block w-full max-w-md mx-auto md:mt-0 lg:max-w-2xl">
            </div>
        </div>
    </div>
    )
}

export default NotFound;