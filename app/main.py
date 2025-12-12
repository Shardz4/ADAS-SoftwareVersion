import cv2
import numpy as np
import adas_pilot

def main():
    cap = cv2.VideoCapture("assets/videos")
    if not cap.isOpened():
        print("error: Could not open the video file. Ensue that it exists.")
        return
    
    while cap.isOpened():
        ret, frame = cap.read()
        if not ret: 
            break

        #Resize for consistent processing (1280x720)
        frame = cv2.resize(frame, (1280, 720))

        # Calling rust lib for lane detect

        try:
            lines = adas_pilot.detect_lanes_rust(frame)
            if lines.shape[0] >= 2:
                for i in range(min(2, lines.shape[0])):
                    x1, y1, x2, y2 = map(int, lines[i])
                    cv2.line(frame, (x1, y1), (x2, y2), (0, 255, 0), 10)  # Green lines for lanes
        
        except Exception as e:
            print(f"Call failed: {e}")

        cv2.inshow('ADAS Pilot - week1: Lane Detection', frame)
        if cv2.waitKey(30) & 0xFF == ord('q'):
            break
    cap.release()
    cv2.destroyAllWindows()

if __name__ == "__main__":
    main()
    