import { createRoot } from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import App from "./App.tsx";
import { MainMusicContainer } from "./components/MainMusicContainer/MainMusicContainer.tsx";
import { MainTable } from "./components/MainTable/MainTable.tsx";
import { UploadBox } from "./components/UploadBox/UploadBox.tsx";
import "./index.css";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    children: [
      {
        path: "upload",
        element: <UploadBox />,
      },
      {
        path: "gallery",
        children: [
          {
            index: true,
            element: <MainTable />,
          },
          {
            path: ":id",
            element: <MainMusicContainer />,
          },
        ],
      },
    ],
  },
]);

createRoot(document.getElementById("root")!).render(
  <RouterProvider router={router} />
);
