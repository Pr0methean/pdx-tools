import { Drawer } from "antd";
import {
  closeDrawerPropagation,
  useSideBarContainerRef,
} from "../../components/SideBarContainer";
import { useSaveFilename } from "../../store";
import { WatchContent } from "./WatchContent";

type WatchDrawerProps = {
  open: boolean;
  closeDrawer: () => void;
};

export const WatchDrawer = ({ open, closeDrawer }: WatchDrawerProps) => {
  const sideBarContainerRef = useSideBarContainerRef();
  const filename = useSaveFilename();
  return (
    <Drawer
      title={`Watch ${filename} for changes`}
      placement="right"
      closable={true}
      mask={false}
      maskClosable={false}
      onClose={closeDrawerPropagation(closeDrawer, open)}
      open={open}
      width="min(800px, 100%)"
    >
      <div className="flex flex-col gap-2" ref={sideBarContainerRef}>
        <WatchContent />
      </div>
    </Drawer>
  );
};
