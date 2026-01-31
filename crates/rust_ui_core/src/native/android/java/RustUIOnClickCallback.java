import android.view.View.OnClickListener;
import android.view.View;

public class RustUIOnClickCallback implements OnClickListener {
    private long rawFunctionBox;

    public RustUIOnClickCallback(){
        this.rawFunctionBox = 0;
    }
    /// consume the rawFunctionBox making future onClick calls panic
    public native void destroy();
    /// the implementor
    @Override
    public native void onClick(View view);
}
