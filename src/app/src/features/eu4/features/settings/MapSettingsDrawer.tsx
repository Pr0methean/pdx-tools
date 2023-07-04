import { Drawer } from "antd";
import {
  closeDrawerPropagation,
  useSideBarContainerRef,
} from "../../components/SideBarContainer";
import { MapSettings } from "./MapSettings";

type MapSettingsDrawerProps = {
  open: boolean;
  closeDrawer: () => void;
};

export const MapSettingsDrawer = ({
  open,
  closeDrawer,
}: MapSettingsDrawerProps) => {
  const sideBarContainerRef = useSideBarContainerRef();
  return (
    <Drawer
      title="Map Settings"
      placement="right"
      closable={true}
      mask={false}
      maskClosable={false}
      onClose={closeDrawerPropagation(closeDrawer, open)}
      open={open}
      width="min(400px, 100%)"
    >
      <div className="flex flex-col gap-2" ref={sideBarContainerRef}>
        <MapSettings />
      </div>
    </Drawer>
  );
};
