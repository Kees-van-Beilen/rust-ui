import android.text.TextWatcher;
import android.text.Editable;
import java.lang.CharSequence;
import android.content.DialogInterface.OnDismissListener;
import android.content.DialogInterface;

public class RustUIOnDismiss implements OnDismissListener {
    private long rawFunctionBox;
    /// consume the rawFunctionBox making future onClick calls panic
    public native void destroy();
    /// the implementor
    @Override
    public native void onDismiss(DialogInterface dialog);
}
